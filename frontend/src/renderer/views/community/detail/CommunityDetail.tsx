import { useEffect } from "react";
import useViewModel from "./ViewModel";
import TextInput from "../../../components/TextInput";
import { useNavigate, useParams } from "react-router-dom";
import Button from "../../../components/button";


export default function CommunityDetail() {
  const navigate = useNavigate();
  const { id } = useParams();
  const { name, price, getCommunity, onChange, updateCommunity, deleteCommunity } =
    useViewModel();

  useEffect(() => {
    getCommunity(id!);
  }, []);

  return (
    <div className="page">
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          padding: 10,
        }}
      >
        <h2>Edit Community</h2>
        <div style={{ display: "flex" }}>
          <Button
            title="Delete"
            onClick={() => {
              deleteCommunity(id!);
              navigate(-1);
            }}
          />

          <Button
            title="Update"
            onClick={() => {
              updateCommunity(id!);
              navigate(-1);
            }}
          />
        </div>
      </div>
      <div style={{ display: "flex", flexDirection: "column", padding: 30 }}>
        <TextInput
          placeholder="Community Name"
          autoFocus={true}
          value={name}
          onChange={(e) => onChange(e.target.value, "name")}
        />
        <TextInput
          placeholder="Community Price"
          type="number"
          value={price}
          onChange={(e) => onChange(e.target.value, "price")}
        />
      </div>
    </div>
  );
}
