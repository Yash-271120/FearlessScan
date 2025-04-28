import { invoke } from "@tauri-apps/api/core"
import { DirectoryPath } from "./types"

export const readPath = async (path: string,mountPoint:string) => {
  const data = await invoke<DirectoryPath[]>("read_directory",{
    path,
    mountPoint
  })

  return data
}
