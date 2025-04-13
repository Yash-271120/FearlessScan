import { DirectoryPath } from "@/types";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faFile, faFolder } from "@fortawesome/free-solid-svg-icons";
import { useNavigationStore } from "@/store/navigation";
import { toast } from "sonner";

type Props = {
  content: DirectoryPath
}
const DirectoryComponent = ({ content }: Props) => {
  const { pushToHistory } = useNavigationStore();

  const handleDirectoryClick = async () => {
    try {
      if (content.type === "file") return;
      pushToHistory(content.path)
    } catch (err) {
      toast.error(err)
    }
  }
  return <div className="rounded-sm hover:bg-gray-500 w-full p-2 flex gap-2 cursor-pointer" onClick={handleDirectoryClick}>
    <div>
      {
        content.type === "file" ? <FontAwesomeIcon icon={faFile} color="#ffffff" /> : <FontAwesomeIcon icon={faFolder} color="#ffd11a" />
      }
    </div>
    <div className="text-white">
      {content.name}
    </div>
  </div>;
};

export default DirectoryComponent;
