type ListProps = {
  message: string;
};

export default function Error({ message }: ListProps): JSX.Element {
  return (
    <div className="mt-1 mx-4 px-4 rounded-md border-l-4 border-red-500 bg-red-50 md:max-w-2xl md:mx-auto md:px-8">
      <div className="flex justify-between py-1">
        <div className="flex">
        <div className="flex items-center">
            <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4 text-red-500" viewBox="0 0 20 20" fill="currentColor">
              <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
            </svg>
          </div>

          <div className="flex justify-between text-sm py-1 ml-3">
            <p className="text-red-600 mt-1">{message}</p>
          </div>
        </div>
      </div>
    </div>
  );
}
