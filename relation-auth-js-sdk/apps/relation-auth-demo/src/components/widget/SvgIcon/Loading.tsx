import SvgIcon from '.'
import './loading.css'

const Loading = (props: any) => {
  return (
    <SvgIcon
      {...props}
      className={`loading-icon ${props.className || ''}`}
      name="loading"
    />
  )
}

export default Loading
