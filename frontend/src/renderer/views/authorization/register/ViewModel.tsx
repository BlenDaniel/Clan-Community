import { useState } from "react";
import * as Yup from "yup";
import AuthService from "../../../../services/auth/AuthService";
import { useNavigate } from 'react-router-dom';


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
  checkCurrentUser:() => void;
}

export default function  RegisterViewModel(): RegisterViewModelInterface {
  const navigate = useNavigate();
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
        .then((response: any) => {
          window.location.reload();
          setSuccessful(true);
          setMessage(JSON.stringify(response));
          console.log(response)
          resolve(response);
          
        })
        .catch((error: any) => {
          setSuccessful(false);
          setMessage(
            error.toString()
          );
          reject(error);
        });
    });
  };
  const checkCurrentUser = () => {
    const currentUser = AuthService.getCurrentUser();

    if (currentUser) {
      navigate(`/community/new`);
    }
  };
  
  return {
    initialValues,
    successful,
    message,
    validationSchema,
    handleRegister,
    checkCurrentUser,
  };
}

