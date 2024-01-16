import './index.css'

const SvgIcon = ({
  name = '',
  prefix = 'icon',
  color = 'currentColor',
  ...props
}) => {
  const symbolId = `#${prefix}-${name}`
  return (
    <svg
      {...props}
      className={`svg-icon ${props.className || ''}`}
      aria-hidden="true"
    >
      <use href={symbolId} fill={color} />
    </svg>
  )
}

export default SvgIcon
