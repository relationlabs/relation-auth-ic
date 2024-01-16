import { useState, useEffect, useCallback } from 'react'

import Button from '../../widget/Button'
import LoadingIcon from '../../widget/SvgIcon/Loading'
import './index.css'

import { authByMetamask } from '@relationlabs/auth'

const LoginButton = ({ onSuccess }: { onSuccess?: Function }) => {
    const [loggedIn, setLoggedIn] = useState(false)
    const [loading, setLoading] = useState(false)
    const [address, setAddress] = useState<string | undefined>()
    useEffect(() => {
        checkStoredToken()
    }, [])

    const checkStoredToken = useCallback(() => {
        const _token = localStorage.getItem('token')
        const _address = localStorage.getItem('address')
        if (_token && _address) {
            setAddress(_address)
            setLoggedIn(true)
            if (typeof onSuccess === 'function') onSuccess(_address)
        } else {
            setLoggedIn(false)
        }
    }, [])

    const requestEthAccount = useCallback(async () => {
        if (typeof window.ethereum !== 'undefined') {
            console.log('MetaMask is installed!')
            const accounts = await window.ethereum.request({
                method: 'eth_requestAccounts',
            })
            const account = accounts[0]
            console.log(account)
            return account
        }
    }, [])

    const login = useCallback(async () => {
        setLoading(true)
        const authResult = await authByMetamask()
        if (!authResult.error && authResult.token) {
            const token = authResult.token
            if (token) {
                const payload = token.substring(token.indexOf('.') + 1, token.lastIndexOf('.'))
                const jwt = atob(payload)
                // todo: do your biz

                const addresss = JSON.parse(JSON.parse(jwt)['sub'])['address']
                localStorage.setItem('token', token)
                localStorage.setItem('address', addresss)
                setAddress(addresss)
                setLoggedIn(true)
                if (typeof onSuccess === 'function') onSuccess(addresss)
            }
        }
        setLoading(false)
    }, [])
    return (
        <div className="login-btn">
            {loggedIn ? (
                <div className="eth-address">{address}</div>
            ) : (
                <Button onClick={login}>
                    {loading ? (
                        <>
                            <LoadingIcon style={{ color: '#fff', marginRight: 8 }} />
                            loading...
                        </>
                    ) : (
                        'Login With Metamask'
                    )}
                </Button>
            )}
        </div>
    )
}

export default LoginButton
