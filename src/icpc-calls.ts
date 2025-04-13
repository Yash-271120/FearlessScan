import { invoke } from "@tauri-apps/api/core"
import { DirectoryPath } from "./types"

export const readPath = async (path: string) => {
  const data = await invoke<DirectoryPath[]>("read_directory",{
    path
  })

  return data
}
