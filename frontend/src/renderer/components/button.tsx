
type ListProps = {
  onClick: () => void;
  title: string;
};

export default function List({ onClick, title }: ListProps): JSX.Element {
  return (
    <button className="w-30 px-4 py-2 text-white font-medium bg-indigo-600 hover:bg-indigo-500 active:bg-indigo-600 rounded-lg duration-150" onClick={onClick}>
      {title}
    </button>
  );
}
