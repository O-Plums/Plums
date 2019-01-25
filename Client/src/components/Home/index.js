import React from 'react'
import { StyleSheet, Text, View } from 'react-native'
import { Button, InputItem, Radio, Icon, Toast, Modal, Flex, } from '@ant-design/react-native'
import { Actions } from 'react-native-router-flux'


export default class Home extends React.Component {
  state = {
    phoneNumber: '',
  }
  onButtonClick4 = () => {
    Modal.prompt(
      'Check phone number',
      'code',
      password => {
        // Add title dynamique
        Actions.preferences({ title: 'Preferences' })
      },
      'secure-text',
      'defaultValue',
    )
  }
  startPreferences = () => {
    const { phoneNumber } = this.state

    if (phoneNumber === '') {
      Toast.fail('Please provide a phone number', 2)
    } else {
      this.setState({ visible2: true })
      this.onButtonClick4()
    }
  }
  render () {
    return (
      <View style={styles.container}>
        <InputItem
          clear
          error
          value={this.state.phoneNumber}
          onChange={value => {
            this.setState({ phoneNumber: value })
          }}
          extra=''
          placeholder='Phone number'
        >
        +33
        </InputItem>
        <Button type='primary' onPress={this.startPreferences} style={{ marginTop: 50 }}>Let's Go</Button>
        <Text onPress={this.onButtonClick4} style={{ marginTop: 10, fontSize: 13, textDecorationLine: 'underline' }}> I already have a code</Text>

      </View>
    )
  }
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
  },
})
