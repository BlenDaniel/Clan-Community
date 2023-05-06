import { useState } from "react";
import { getCommunitiesUseCase } from "../../../../services/community/Get";

interface CommunityListViewModel {
    error: string;
    Communities: Community[];
    getCommunities: () => Promise<void>;
}

export default function useCommunityListViewModel(): CommunityListViewModel {
    const [error, setError] = useState<string>("");
    const [Communities, setCommunities] = useState<Community[]>([]);

    async function getCommunities() {
        const { result, error } = await getCommunitiesUseCase();
        setError(error!);
        setCommunities(result!);
        console.log(result!);
    }

    return {
        error,
        Communities,
        getCommunities,
    };
}
