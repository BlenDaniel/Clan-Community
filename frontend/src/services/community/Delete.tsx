import { deleteCommunity } from "../../data/repository/CommunityRepository";


export async function DeleteCommunityUseCase(id: string) {
    return await deleteCommunity(id)
}