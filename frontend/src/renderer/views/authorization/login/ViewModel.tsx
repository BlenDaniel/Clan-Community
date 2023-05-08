import {  useState } from "react";
import * as Yup from "yup";

import AuthService from "../../../../services/auth/AuthService";
import { useNavigate } from 'react-router-dom';

interface LoginViewModel {
  loading: boolean;
  message: string;
  validationSchema: Yup.ObjectSchema<any>;
  handleLogin: (formValue: { email: string; password: string }) => void;
  checkCurrentUser: () => void;
}

export default function LogInViewModel(): LoginViewModel {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState("");

  

  const validationSchema = Yup.object().shape({
    email: Yup.string().required("This field is required!"),
    password: Yup.string().required("This field is required!"),
  });

  const handleLogin = (formValue: { email: string; password: string }) => {
    const { email, password } = formValue;

    setMessage("");
    setLoading(true);

    AuthService.login(email, password)
      .then(() => {
        window.location.reload();
        navigate(`/community/new`);
      })
      .catch((error: any) => {
        const resMessage =
          (error.response && error.response.data && error.response.data.message) ||
          error.message ||
          error.toString();

        setLoading(false);
        setMessage(resMessage);
      });
  };

  const checkCurrentUser = () => {
    const currentUser = AuthService.getCurrentUser();

    if (currentUser) {
      navigate(`/community/new`);
    }
  };

  return {
    loading,
    message,
    validationSchema,
    handleLogin,
    checkCurrentUser,
  };
}

