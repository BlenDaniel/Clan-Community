const COLLECTION = "COMMUNITIES";

export function getAll(): Promise<{
  error: string | null;
  result: Array<any>;
}> {
  try {
    let data: Array<any> = [];
    const dataString = window.localStorage.getItem(COLLECTION);
    if (dataString) {
      data = JSON.parse(dataString);
    }
    return Promise.resolve({ error: null, result: data });
  } catch (err: any) {
    return Promise.resolve({ error: err.message, result: [] });
  }
}

export function getOne(
  id: string
): Promise<{ error: string | null; result: any | null }> {
  try {
    let data: Array<any> = [];
    const dataString = window.localStorage.getItem(COLLECTION);
    if (dataString) {
      data = JSON.parse(dataString);
    }
    const filteredData = data.filter((item) => item.id === id);

    return Promise.resolve({
      error: null,
      result: filteredData.length > 0 ? filteredData[0] : null,
    });
  } catch (err: any) {
    return Promise.resolve({ error: err.message, result: null });
  }
}

export async function create(communityData: CommunityData): Promise<{ error: string | null; result: boolean }> {
  const { error, result } = await getAll();
  const data = result;

  const communityNew: Community = {
    id: new Date().getTime().toString(),
    name: communityData.name,
    price: communityData.price
  }

  data.push(communityNew);
  window.localStorage.setItem(COLLECTION, JSON.stringify(data));
  return Promise.resolve({ error: null, result: true });
}

export async function deleteOne(
  id: string
): Promise<{ error: string | null; result: boolean }> {
  const { error, result } = await getAll();
  const data = result;
  const deleteIndex = data.findIndex((item) => item.id === id);
  data.splice(deleteIndex, 1);
  window.localStorage.setItem(COLLECTION, JSON.stringify(data));
  return Promise.resolve({ error: null, result: true });
}

export async function update(
  id: string,
  CommunityData: { name: string; price: number }
): Promise<{ error: string | null; result: boolean }> {
  const { error, result } = await getAll();
  const data = result;

  data.forEach((item) => {
    if (item.id === id) {
      item.name = CommunityData.name;
      item.price = CommunityData.price;
    }
  });
  window.localStorage.setItem(COLLECTION, JSON.stringify(data));
  return Promise.resolve({ error: null, result: true });
}
