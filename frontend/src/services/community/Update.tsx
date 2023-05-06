import { updateCommunity } from "../../data/repository/CommunityRepository";

export async function UpdateCommunityUseCase(id: string, communityData: Community) {
    return await updateCommunity(id, communityData)
}