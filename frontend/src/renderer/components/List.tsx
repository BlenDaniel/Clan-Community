type ListProps = {
  data?: { id: string; name: string; price: number }[];
  onRowClick: (id: string) => void;
};

export default function List({ data = [], onRowClick }: ListProps): JSX.Element {
  return (
    <div style={{ padding: 20 }}>
      {data.map((item) => {
        return (
          <div
            key={item.id}
            style={{
              cursor: "pointer",
              display: "flex",
              justifyContent: "space-between",
              padding: 20,
              border: "1px solid black",
              margin: 10,
            }}
            onClick={() => onRowClick(item.id)}
          >
            <div>{item.name}</div>
            <div>{item.price}</div>
          </div>
        );
      })}
    </div>
  );
}
