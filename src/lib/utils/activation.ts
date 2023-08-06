import { toast } from "svoast";
import api from "./api";
import { invoke } from "@tauri-apps/api";

export async function activateKey(key: string) {
    if (!key) return;

    if (!(await validateKey(key))) {
        return toast.error("Invalid key", {
            closable: true,
            duration: 5000,
        });
    }

    const deviceFingerprint: string = await invoke("get_device_fingerprint")

    if (! await checkKeyUsage(key, deviceFingerprint)) {
        return toast.error("Key expired", {
            closable: true,
            duration: 5000,
        })
    }

    await invoke("save_activation", { key });
    await invoke("activation_complete");
}

async function validateKey(key: string) {
    try {
        const res = await api.get(`/activation/${key}`);
        return res.status == 200;
    } catch (err) {
        return false;
    }
}

async function checkKeyUsage(key: string, device: string) {
    try {
        const res = await api.post(`/activation/${key}/usage`, {
            device: device
        })
        return res.status == 200
    } catch (err) {
        return false
    }
}