import { useEffect } from "react";
import useViewModel from "./ViewModel";
import TextInput from "../../../components/TextInput";
import { useNavigate } from "react-router-dom";
import Button from "../../../components/button";

export default function CommunityNew() {
  const navigate = useNavigate();
  const { name, price, onChange, saveCommunity } = useViewModel();

  useEffect(() => {return; }, []);

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
        <h2>New Community</h2>
        <Button
          title={"Save"}
          onClick={() => {
            saveCommunity();
            navigate(-1);
          }}
        />
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
