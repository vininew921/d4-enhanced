import { invoke } from "@tauri-apps/api";
import type { DiabloItem } from "./models";

export const api_get_items = async (): Promise<Array<DiabloItem>> => {
  let result: Array<DiabloItem>;
  await invoke("get_items").then((res: Array<DiabloItem>) => {
    result = res;
  });

  return result;
};

export const api_remove_item = async (id: number): Promise<null> => {
  await invoke("get_items", { id: id });

  return null;
};

export const api_remove_all_items = async (): Promise<null> => {
  await invoke("remove_all_items");

  return null;
};

export const api_screenshot_item = async (): Promise<null> => {
  await invoke("screenshot_item");

  return null;
};
