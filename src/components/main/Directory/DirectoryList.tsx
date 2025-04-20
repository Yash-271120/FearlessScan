import { useDirectoryStore } from "@/store/directory";
import DirectoryComponent from "@/components/main/Directory/DirectoryComponent";
import SearchComponent from "@/components/main/Directory/SearchComponent";
import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { FSEvent } from "@/types";
import { useSearchStore } from "@/store/search";

const FS_EVENT_NAME = "fs-event";

const DirectoryList = () => {
  const { directory, addItem, removeItem } = useDirectoryStore();
  const {searchData} = useSearchStore();

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
    searchData.length > 0 ? searchData.map((item, idx) => {
      return <SearchComponent key={idx} data={item} />
    }) :
    directory.map((item, idx) => {
      return <DirectoryComponent key={idx} content={item} />
    })
  }</div>
};

export default DirectoryList;
