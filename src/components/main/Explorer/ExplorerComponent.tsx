import { Button } from "@/components/ui/button";
import DirectoryList from "../Directory/DirectoryList";
import { useNavigationStore } from "@/store/navigation";

const ExplorerComponent = () => {
  const {canGoForward, canGoBack, goForward, goBack} = useNavigationStore();

  return <div>
    <div className="flex flex-row justify-center fixed top-0 left-0 right-0 w-full bg-gray-700">
      <div>
        <Button disabled={!canGoBack()} onClick={goBack}>Back</Button>
        <Button disabled={!canGoForward()} onClick={goForward}>Forward</Button>
      </div>
    </div>

    <div>
      <DirectoryList />
    </div>
  </div>;
};


export default ExplorerComponent;
