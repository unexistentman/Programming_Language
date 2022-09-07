import { createRef, useState } from "react";
import '../style/Validation.css'

function Validation() {
  const [form, setForm] = useState({
    password: '',
    clicked: false,
    validated: false
  });
  const {password, clicked, validated} = form;
  const onChange = event => {
    const nextFrom = {
      ...form,
      password: event.target.value
    };
    setForm(nextFrom);
  };
  const input = createRef();
  const onClick = () => {
    const nextFrom = {
      ...form,
      clicked: true,
      validated: password === '0000'
    };
    setForm(nextFrom);
    input.current.focus();
  };

  return (
    <div className="validation">
      <input
        ref={input}
        type="password"
        value={password}
        onChange={onChange}
        className={clicked ? (validated ? 'success validationInput' : 
          'failure validationInput') : 'validationInput'}
      />
      <button onClick={onClick}>validate</button>
    </div>
  );
}

export default Validation;