import { useDirectoryStore } from "@/store/directory";
import DirectoryComponent from "@/components/main/Directory/DirectoryComponent";
import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { FSEvent } from "@/types";

const FS_EVENT_NAME = "fs-event";

const DirectoryList = () => {
  const { directory, addItem, removeItem } = useDirectoryStore();

  let render = 0;
  useEffect(() => {

    if (render === 0) {
      listen<FSEvent>(FS_EVENT_NAME, (event) => {
        const payload = event.payload;
        switch (payload.kind) {
          case "create":
            addItem(payload.directoryPath);
            break;
          case "remove":
            removeItem(payload.directoryPath);
            break;
          default:
            break;
        }
      })
      render++;
    }
  }, [])
  return <div>{
    directory.map((item, idx) => {
      return <DirectoryComponent key={idx} content={item} />
    })
  }</div>
};

export default DirectoryList;
