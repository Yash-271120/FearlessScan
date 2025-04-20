import { Button } from "@/components/ui/button";
import DirectoryList from "../Directory/DirectoryList";
import { useNavigationStore } from "@/store/navigation";
import { Input } from "@/components/ui/input";
import { useState } from "react";
import { toast } from "sonner";
import { invoke } from "@tauri-apps/api/core";
import { useSearchStore } from "@/store/search";
import { SearchResult } from "@/types";

const ExplorerComponent = () => {
  const { canGoForward, canGoBack, goForward, goBack, getCurrentPath } = useNavigationStore();
  const {setSearchData} = useSearchStore();
  const [searchTerm, setSearchTerm] = useState<string>("");
  const [isSearching, setIsSearching] = useState<boolean>(false);


  const handleSearchClick = async () => {
    try {
      setIsSearching(true);
      const data = await invoke("search_directory", {
        path: getCurrentPath(),
        query: searchTerm,
      });

      setSearchData(data as SearchResult[]);
    } catch (err) {
      toast.error("Error searching");
    } finally {
      setIsSearching(false);
    }
  };

  return <div>
    <div className="flex flex-row justify-between fixed top-0 left-0 right-0 w-full bg-gray-700 p-2">
      <div>
        <Button disabled={!canGoBack()} onClick={goBack}>Back</Button>
        <Button disabled={!canGoForward()} onClick={goForward}>Forward</Button>
      </div>
      <div className="flex flex-row gap-2">
        <Input placeholder="Search" value={searchTerm} onChange={(e) => setSearchTerm(e.target.value)} />
        <Button onClick={handleSearchClick} disabled={isSearching}>Search</Button>
      </div>
    </div>

    <div className="mt-10">
      <DirectoryList />
    </div>
  </div>;
};


export default ExplorerComponent;
