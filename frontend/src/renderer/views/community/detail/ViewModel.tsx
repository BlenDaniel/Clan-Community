import { useState } from "react";
import { GetCommunityUseCase } from "../../../../services/community/CRUD";
import { UpdateCommunityUseCase } from "../../../../services/community/CRUD";
import { DeleteCommunityUseCase } from "../../../../services/community/CRUD";


interface CommunityDetailViewModel {
  error: string;
  id: string | null;
  name: string;
  price: number;
  getCommunity: (id: string) => Promise<void>;
  onChange: (value: string | string, prop: keyof Community) => void;
  updateCommunity: (id: string) => Promise<void>;
  deleteCommunity: (id: string) => Promise<void>;
}

export default function CommunityDetailViewModel(): CommunityDetailViewModel {
  const [error, setError] = useState("");
  const [values, setValues] = useState<Community>({
    id: "",
    name: "",
    price: 0,
  });

  async function getCommunity(id: string): Promise<void> {
    const { result, error } = await GetCommunityUseCase(id);
    setError(error!);
    setValues({ ...result! });
  }

  function onChange(
    value: string | number,
    prop: keyof Community
  ): void {
    setValues({ ...values, [prop]: value });
  }

  async function updateCommunity(id: string): Promise<void> {
    const { error } = await UpdateCommunityUseCase(id, values);
    setError(error!);
  }

  async function deleteCommunity(id: string): Promise<void> {
    const { error } = await DeleteCommunityUseCase(id);
    setError(error!);
  }

  return {
    error,
    deleteCommunity,
    updateCommunity,
    getCommunity,
    onChange,
    ...values,
  };
}
