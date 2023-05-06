import { createCommunity } from "../../data/repository/CommunityRepository";


export async function CreateCommunityUseCase(communityData: CommunityData) {
    return await createCommunity(communityData);
  }