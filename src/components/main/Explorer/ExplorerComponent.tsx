import { Button } from "@/components/ui/button";
import DirectoryList from "../Directory/DirectoryList";
import { useNavigationStore } from "@/store/navigation";
import { Input } from "@/components/ui/input";
import { useEffect, useState } from "react";
import { toast } from "sonner";
import { invoke } from "@tauri-apps/api/core";
import { useSearchStore } from "@/store/search";
import { SearchResult } from "@/types";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faMagnifyingGlass, faCircleXmark,faArrowLeft, faArrowRight } from "@fortawesome/free-solid-svg-icons";
import { useDirectoryStore } from "@/store/directory";

const ExplorerComponent = () => {
  const { canGoForward, canGoBack, goForward, goBack, getCurrentPath } = useNavigationStore();
  const { setSearchData, clearSearchData, isSearching } = useSearchStore();
  const { currentMountPoint } = useDirectoryStore();
  const [searchTerm, setSearchTerm] = useState<string>("");


  const handleSearchClick = async () => {
    try {
      const queryToSearch = searchTerm.trim();
      if(queryToSearch.length === 0){
        return;
      }
      const data = await invoke("search_directory_fast", {
        dirPath: getCurrentPath(),
        query: queryToSearch,
        mountPoint: currentMountPoint,
      })
      setSearchData(data as SearchResult[]);
    } catch (err) {
      toast.error(err);
    }
  };

  const handleClearClick = () => {
    setSearchTerm("");
    clearSearchData();
  };

  const handleGoBack = () => {
    goBack();
    setSearchTerm("");
    clearSearchData();
  }

  const handleGoForward = () => {
    goForward();
    setSearchTerm("");
    clearSearchData();
  }

  return <div>
    <div className="flex flex-row justify-between fixed top-0 left-0 right-0 w-full bg-gray-700 p-2">
      <div className="flex flex-row gap-2">
        <Button disabled={!canGoBack()} onClick={handleGoBack}>
          <FontAwesomeIcon icon={faArrowLeft} color="#999999" />
        </Button>
        <Button disabled={!canGoForward()} onClick={handleGoForward}>
          <FontAwesomeIcon icon={faArrowRight} color="#999999" />
        </Button>
      </div>
      <div className="flex flex-row gap-2 items-center border-1 rounded-sm px-2">
        <FontAwesomeIcon icon={faMagnifyingGlass} color="#999999" />
        <Input className="focus-visible:outline-none focus-visible:border-none focus-visible:ring-0 ring-0 border-none" placeholder="Search" value={searchTerm} onChange={(e) => setSearchTerm(e.target.value)} />
        {
          isSearching ? <FontAwesomeIcon icon={faCircleXmark} color="#999999" onClick={handleClearClick} /> : <Button className="cursor-pointer" onClick={handleSearchClick}>Search</Button>
        }
      </div>
    </div>

    <div className="mt-10">
      <DirectoryList />
    </div>
  </div>;
};


export default ExplorerComponent;
