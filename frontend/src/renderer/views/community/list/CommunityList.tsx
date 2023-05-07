import  { useEffect } from "react"
import useViewModel from "./ViewModel"
import List from "../../../components/List"
import { useNavigate } from "react-router-dom";
import Button from "../../../components/button";

export default function CommunityList() {
    const navigate = useNavigate();
    const { Communities, getCommunities} = useViewModel();

    useEffect(() => {
        getCommunities()
    }, [])

    return (
        <div className="page">
            <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", padding: 10 }}>
                <h2>Community List</h2>
                <Button title={"New"} onClick={() => navigate(`/community/new`)} />
            </div>
            <List data={Communities} onRowClick={(id: string) => navigate(`/community/detail/${id}`)} />
        </div>
    );
}
