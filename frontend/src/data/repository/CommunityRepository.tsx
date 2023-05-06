import { create, update, getAll, getOne, deleteOne } from "../api/CommunityLocalStorageDataSource";


interface Result<T> {
  result?: T;
  error?: string | undefined | null;
}

export async function createCommunity(data: CommunityData): Promise<Result<boolean>> {
  const { result, error } = await create(data);
  return { result, error };
}

export async function deleteCommunity(id: string): Promise<Result<boolean>> {
  const { result, error } = await deleteOne(id);
  return { result, error };
}


// Should only update some parts of the JSON not all
export async function updateCommunity(id: string, data: Community): Promise<Result<boolean>> {
  const { result, error } = await update(id, data);
  return { result, error };
}

export async function getCommunities(): Promise<Result<Array<Community>>> {
  const { result, error } = await getAll();
  return { result, error };
}

export async function getCommunity(id: string): Promise<Result<Community>> {
  const { result, error } = await getOne(id);
  return { result, error };
}
