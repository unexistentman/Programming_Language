import '../style/Counter.css';
import { useReducer } from 'react';

function reducer(state, action) {
  switch (action.type) {
    case 'INCREMENT':
      return {value: state.value + 1};
    case 'DECREMENT':
      return {value: state.value - 1};
    default:
      return state;
  }
}

function Counter() {
  const [state, dispatch] = useReducer(reducer, {value: 0});

  return (
    <div className="counter">
      <div>{state.value}</div>
      <div>
        <button className="counterBtn" onClick={() => dispatch({type: 'INCREMENT'})}>
          +
        </button>
        <button className="counterBtn" onClick={() => dispatch({type: 'DECREMENT'})}>
          -
        </button>
      </div>
    </div>
  );
}

export default Counter;