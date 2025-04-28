import { useVirtualizer, useWindowVirtualizer } from "@tanstack/react-virtual";
import { useSearchStore } from "@/store/search";
import { useRef } from "react";
import SearchComponent from "./SearchComponent";

const SearchVirtualList = () => {
  const { searchData } = useSearchStore();
  const parentRef = useRef<HTMLDivElement>(null);
  const count = searchData.length;

  if(count === 0){
    return <div className="text-white">No results found</div>
  }

  const virtualizer = useVirtualizer({
    count,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 30,
    enabled: true,
  });

  const items = virtualizer.getVirtualItems();
  return (
    <div>
      <div
        ref={parentRef}
        className="List absolute top-15 left-0 bottom-0 right-0"
        style={{
          overflowY: "auto",
          contain: "strict",
        }}
      >
        <div
          style={{
            height: virtualizer.getTotalSize(),
            width: "100%",
            position: "relative",
          }}
        >
          <div
            style={{
              position: "absolute",
              top: 0,
              left: 0,
              width: "100%",
              transform: `translateY(${items[0]?.start ?? 0}px)`,
            }}
          >
            {items.map((virtualRow) => (
              <div
                key={virtualRow.key}
                data-index={virtualRow.index}
                ref={virtualizer.measureElement}
                className={
                  virtualRow.index % 2 ? "ListItemOdd" : "ListItemEven"
                }
              >
                <div style={{ padding: "10px 0" }}>
                  <SearchComponent data={searchData[virtualRow.index]}/>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

export default SearchVirtualList;
