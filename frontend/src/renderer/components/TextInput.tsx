import { InputHTMLAttributes } from "react";

type TextInputProps = InputHTMLAttributes<HTMLInputElement>;

export default function TextInput(props: TextInputProps): JSX.Element {
  return (
    <input
      style={{ height: 28, padding: 10, marginBottom: 10 }}
      {...props}
    />
  );
}
