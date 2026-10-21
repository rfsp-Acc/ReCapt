import { useCallback, useEffect, useState } from "react";
import { api, InputDevice } from "../lib/tauri";

export function useInputDevices() {
  const [devices, setDevices] = useState<InputDevice[]>([]);
  const [selected, setSelected] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(() => {
    api
      .listInputDevices()
      .then((r) => {
        setDevices(r.devices);
        setSelected(r.selected);
      })
      .catch((e) => setError(String(e)));
  }, []);

  useEffect(refresh, [refresh]);

  const select = async (id: string | null) => {
    setError(null);
    try {
      await api.setInputDevice(id);
      setSelected(id);
    } catch (e) {
      setError(String(e));
    }
  };

  return { devices, selected, error, refresh, select };
}
