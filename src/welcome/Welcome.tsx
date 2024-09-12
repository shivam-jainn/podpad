import { Button } from "antd";
import { useNavigate } from "react-router-dom";

export default function Welcome() {
  const navigate = useNavigate();

  const handleProceedToEditor = () => {
    navigate("/editor");
  };

  return (
    <div>
      <div>Welcome to podspace</div>
      <Button type="primary" onClick={handleProceedToEditor}>
        Proceed to Editor
      </Button>
    </div>
  );
}
