#include <Windows.h>

#include <CommCtrl.h>
#include <endpointvolume.h>
#include <mmdeviceapi.h>

#include <spdlog/sinks/stdout_color_sinks.h>
#include <spdlog/spdlog.h>
#include <json.hpp>

#include <algorithm>
#include <fstream>
#include <string>
#include <vector>


#define EXIT_ON_ERROR(hres, cause)                           \
    if (FAILED(hres)) {                                      \
        spdlog::error("Error happened, cause: {}", cause);   \
        return -1;                                           \
    }

using json = nlohmann::json;
using std::accumulate;
using std::ifstream;
using std::string;
using std::transform;
using std::vector;

GUID guid = GUID_NULL;

int main()
{
	HRESULT hr = S_OK;
	IMMDeviceEnumerator* pEnumerator = nullptr;
	IMMDevice* pEndptDevice = nullptr;
	IAudioEndpointVolume* pEndptVol = nullptr;

	hr = CoInitialize(nullptr);
	EXIT_ON_ERROR(hr, "CoInitialize");

	hr = CoCreateGuid(&guid);
	EXIT_ON_ERROR(hr, "CoCreateGuid");

	hr = CoCreateInstance(__uuidof(MMDeviceEnumerator), nullptr, CLSCTX_INPROC_SERVER,
		__uuidof(IMMDeviceEnumerator), (void**)& pEnumerator);
	EXIT_ON_ERROR(hr, "CoCreateInstance");

	hr = pEnumerator->GetDefaultAudioEndpoint(eCapture, eConsole, &pEndptDevice);
	EXIT_ON_ERROR(hr, "pEnumerator->GetDefaultAudioEndpoint");

	hr = pEndptDevice->Activate(__uuidof(IAudioEndpointVolume), CLSCTX_ALL, nullptr, (void**)& pEndptVol);
	EXIT_ON_ERROR(hr, "pEndptDevice->Activate(IAudioEndpointVolume)");

	float level = 0;
	hr = pEndptVol->GetMasterVolumeLevelScalar(&level);
	EXIT_ON_ERROR(hr, "pEndptVol->GetMasterVolumeLevelScalar");

	spdlog::info("Current microphone volume: {}", level * 100);

	json config;
	ifstream file("config.json");
	if (!file.is_open()) {
		spdlog::error("missing config file");
		return -1;
	}

	file >> config;

	auto iter_mods = config.find("mods");
	if (iter_mods == config.end()) {
		spdlog::error("mods missing");
		return -1;
	}

	auto mods = iter_mods->get<vector<string>>();
	vector<int> mod_codes{};

	transform(mods.begin(), mods.end(), std::back_inserter(mod_codes), [](const string & mod) {
		return std::stoi(mod, nullptr, 16);
		});

	int mod_code = accumulate(mod_codes.begin(), mod_codes.end(), 0, [](int a, int b) {
		return a | b;
		});
	mod_code |= MOD_NOREPEAT;

	auto iter_key = config.find("key");
	if (iter_key == config.end()) {
		spdlog::error("key missing");
		return -1;
	}

	auto key = iter_key->get<string>();
	auto key_code = std::stoi(key, nullptr, 16);

	spdlog::debug("Trying to register hotKey");
	if (RegisterHotKey(
		nullptr,
		1,
		mod_code,
		key_code)) {
		spdlog::info("Hotkey registered");
	}
	else {
		spdlog::critical("Hotkey register failed, the hotkey has been used by another program");
		return -1;
	}

	MSG msg{};

	while (GetMessage(&msg, nullptr, 0, 0) != 0) {
		if (msg.message == WM_HOTKEY) {
			spdlog::debug("Hotkey detected");

			BOOL mute;
			pEndptVol->GetMute(&mute);
			pEndptVol->SetMute(!mute, nullptr);

			spdlog::info("Microphone {}", mute ? "unmuted" : "muted");
		}
	}
}
