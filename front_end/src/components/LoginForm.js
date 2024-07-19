import React, { Component } from 'react';
import axios from 'axios';

class LoginForm extends Component {
    state = {
        username: "",
        password: "",
        //server_url: "https://automatic-space-umbrella-jggx9p6px57355qg-3000.app.github.dev/v1/auth/login"
        //server_url: "http://localhost:8000/v1/auth/login"
    }

    submitLogin = (e) => {
        e.preventDefault();
        axios.post("http://localhost:8000/v1/auth/login", 
        //axios.get("http://localhost:8000/v1/auth/login", 
            { 
                "username": this.state.username,
                "password": this.state.password
            }, 
            {
                headers: {
                    "Origin": "*",
                }
            })
        .then((response) => {
            this.setState({ username: "", password: "" });
            this.props.handleLogin(response.data["token"]);
        })
        .catch((error) => {
            alert(error);
            this.setState({password: "", firstName: ""});
        });
    }

    handlepPasswordChange = (e) => {
        this.setState({password: e.target.value});
    }

    handleUsernameChange = (e) => {
        this.setState({username: e.target.value});
    }

    render() {
        return (
            <form className="login" onSubmit={this.submitLogin}>

                <h1 className="login-title">Login</h1>

                <input type="text" className="login-input" 
                    placeholder="Username" 
                    autoFocus onChange={this.handleUsernameChange} 
                    value={this.state.username} />

                <input type="password" className="login-input"
                    placeholder="Password"
                    onChange={this.handlepPasswordChange}
                    value={this.state.password} />

                <input type="submit" value="Lets Go" 
                className="login-button" />
            </form>
        )
    }
}

export default LoginForm;