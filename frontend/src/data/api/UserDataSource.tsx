import User from "../../types/user.type";
import axios from "axios";

const API_URL = "your_api_url";

export async function getAll(): Promise<{ error: string | null; result: User[] | null }> {
  try {
    const response = await axios.get(`${API_URL}/users`);
    const result = response.data;

    return Promise.resolve({ error: null, result: result });
  } catch (error: any) {
    return Promise.resolve({ error: error.message, result: null });
  }
}

export async function checkUserExists(
  id: string
): Promise<{ error: string | null; result: boolean }> {
  try {
    const response = await axios.get(`${API_URL}/users/${id}`);
    const result = response.data;

    return Promise.resolve({ error: null, result: !!result });
  } catch (error: any) {
    return Promise.resolve({ error: error.message, result: false });
  }
}

export async function getOne(
  id: string
): Promise<{ error: string | null; result: User | null }> {
  try {
    const response = await axios.get(`${API_URL}/users/${id}`);
    const result = response.data;

    return Promise.resolve({ error: null, result: result });
  } catch (error: any) {
    return Promise.resolve({ error: error.message, result: null });
  }
}

export async function create(UserData: User): Promise<{ error: string | null; result: boolean }> {
  try {
    return Promise.resolve(await axios.post(`${API_URL}/users`, UserData));
  } catch (error: any) {
    return Promise.resolve({ error: error.message, result: false });
  }
}

export async function deleteOne(
  id: string
): Promise<{ error: string | null; result: boolean }> {
  try {
    await axios.delete(`${API_URL}/users/${id}`);

    return Promise.resolve({ error: null, result: true });
  } catch (error: any) {
    return Promise.resolve({ error: error.message, result: false });
  }
}

export async function update(
  id: string,
  UserData: { name: string; price: number }
): Promise<{ error: string | null; result: boolean }> {
  try {
    await axios.put(`${API_URL}/users/${id}`, UserData);

    return Promise.resolve({ error: null, result: true });
  } catch (error: any) {
    return Promise.resolve({ error: error.message, result: false });
  }
}
