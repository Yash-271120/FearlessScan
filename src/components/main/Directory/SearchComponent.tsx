import { SearchResult } from "@/types"
import { faFile } from "@fortawesome/free-solid-svg-icons"
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome"

type Props = {
  data: SearchResult
}

const SearchComponent = ({ data }: Props) => {

  return <div className="rounded-sm hover:bg-gray-500 w-full p-2 flex gap-2 cursor-pointer">
    <div>
      <FontAwesomeIcon icon={faFile} color="#ffffff" />
    </div>
    <div className="flex flex-col">
      <div className="text-white">
        {
          data.name.split("").map((char, index) => {
            return <span key={index} className={data.indices.includes(index) ? "bg-yellow-500" : ""}>{char}</span>
          })
        }
      </div>
      <div className="text-slate-400">
        {data.path}
      </div>

    </div>
  </div>
}

export default SearchComponent
