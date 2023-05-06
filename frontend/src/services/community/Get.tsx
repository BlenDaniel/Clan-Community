import { getCommunity, getCommunities } from "../../data/repository/CommunityRepository";

export async function GetCommunityUseCase(id: string) {
    return await getCommunity(id)
}

export async function getCommunitiesUseCase() {
    return await getCommunities()
}