import { useState } from "react";
import { CreateCommunityUseCase } from "../../../../services/community/CRUD";

type CommunityNewValues = {
  name: string;
  price: number;
};

export default function CommunityNewViewModel() {
  const [error, setError] = useState("");
  const [values, setValues] = useState<CommunityNewValues>({
    name: "",
    price: 0,
  });

  function onChange(value: string, prop: keyof CommunityNewValues) {
    setValues({ ...values, [prop]: value });
  }

  async function saveCommunity() {
    const {  error } = await CreateCommunityUseCase(values);
    setError(error!);
  }

  return {
    ...values,
    error,
    onChange,
    saveCommunity,
  };
}
