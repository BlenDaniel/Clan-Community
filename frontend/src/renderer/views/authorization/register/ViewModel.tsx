import { useState } from "react";
import * as Yup from "yup";
import AuthService from "../services/auth.service";

interface FormValues {
  username: string;
  email: string;
  password: string;
}

interface RegisterResponse {
  data: {
    message: string;
  };
}

interface RegisterViewModelInterface {
  initialValues: FormValues;
  successful: boolean;
  message: string;
  validationSchema(): Yup.Schema<FormValues>;
  handleRegister(formValue: FormValues): Promise<RegisterResponse>;
}

const RegisterViewModel = (): RegisterViewModelInterface => {
  const [error, setError] = useState("");
  const [successful, setSuccessful] = useState(false);
  const [message, setMessage] = useState("");
  const initialValues: FormValues = {
    username: "",
    email: "",
    password: "",
  };
  const validationSchema = (): Yup.Schema<FormValues> =>
    Yup.object().shape({
      username: Yup.string()
        .test(
          "len",
          "The username must be between 3 and 20 characters.",
          (val) =>
            !!val && val.toString().length >= 3 && val.toString().length <= 20
        )
        .required("This field is required!"),
      email: Yup.string()
        .email("This is not a valid email.")
        .required("This field is required!"),
      password: Yup.string()
        .test(
          "len",
          "The password must be between 6 and 40 characters.",
          (val) =>
            !!val && val.toString().length >= 6 && val.toString().length <= 40
        )
        .required("This field is required!"),
    });

  const handleRegister = (formValue: FormValues): Promise<RegisterResponse> => {
    const { username, email, password } = formValue;

    return new Promise<RegisterResponse>((resolve, reject) => {
      AuthService.register(username, email, password)
        .then((response: { data: { message: any } }) => {
          setSuccessful(true);
          setMessage(response.data.message);
          resolve({ data: { message: response.data.message } });
        })
        .catch((error: any) => {
          setSuccessful(false);
          setMessage(
            error.response?.data?.message || error.message || error.toString()
          );
          reject(error);
        });
    });
  };

  return {
    initialValues,
    successful,
    message,
    validationSchema,
    handleRegister,
  };
};

export default RegisterViewModel;
