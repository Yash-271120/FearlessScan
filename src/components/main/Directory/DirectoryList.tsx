import { useDirectoryStore } from "@/store/directory";
import DirectoryComponent from "@/components/main/Directory/DirectoryComponent";

const DirectoryList = () => {
  const { directory } = useDirectoryStore();
  return <div>{
    directory.map((item, idx) => {
      return <DirectoryComponent key={idx} content={item} />
    })
  }</div>
};

export default DirectoryList;
