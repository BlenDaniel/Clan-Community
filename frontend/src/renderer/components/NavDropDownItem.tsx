import { ReactElement } from "react";

export default function NavDropDownItem({
  children,
  title,
  text,
}: {
  children: ReactElement;
  title: string;
  text: string;
}) {
  return (
    <>
      <a
        href="#"
        className="-m-3 p-3 flex items-start rounded-lg hover:bg-gray-50"
      >
        {children}
        <div className="text-left ml-4">
          <p className="text-base font-medium text-gray-900">{title}</p>
          <p className="mt-1 text-sm text-gray-500">{text}</p>
        </div>
      </a>
    </>
  );
}
