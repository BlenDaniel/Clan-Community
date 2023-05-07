import { createCommunity } from "../../data/repository/CommunityRepository";
import { deleteCommunity } from "../../data/repository/CommunityRepository";
import {
  getCommunity,
  getCommunities,
} from "../../data/repository/CommunityRepository";
import { updateCommunity } from "../../data/repository/CommunityRepository";

export async function CreateCommunityUseCase(communityData: CommunityData) {
  return await createCommunity(communityData);
}

export async function DeleteCommunityUseCase(id: string) {
  return await deleteCommunity(id);
}

export async function GetCommunityUseCase(id: string) {
  return await getCommunity(id);
}

export async function getCommunitiesUseCase() {
  return await getCommunities();
}

export async function UpdateCommunityUseCase(
  id: string,
  communityData: Community
) {
  return await updateCommunity(id, communityData);
}
