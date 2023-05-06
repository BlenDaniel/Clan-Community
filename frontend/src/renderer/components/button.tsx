
type ListProps = {
  onClick: () => void;
  title: string;
};

export default function List({ onClick, title }: ListProps): JSX.Element {
  return (
    <button style={{ height: 24 }} onClick={onClick}>
      {title}
    </button>
  );
}
