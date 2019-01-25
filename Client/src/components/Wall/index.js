import React from 'react'
import { StyleSheet, Text, View } from 'react-native'
import { Button, InputItem, Radio, Icon, Toast, Modal, Flex, Card, WingBlank } from '@ant-design/react-native'
import { Actions } from 'react-native-router-flux'

export default class Wall extends React.Component {
  state = {
  }
  render () {
    return (
      <View style={styles.container}>

          <Card>
            <Card.Header
              title="This is title"
              thumbStyle={{ width: 30, height: 30 }}
              thumb="https://gw.alipayobjects.com/zos/rmsportal/MRhHctKOineMbKAZslML.jpg"
              extra="this is extra"
            />
            <Card.Body>
              <View style={{ height: 80 }}>
                <Text style={{ marginLeft: 1 }}>Card Content</Text>
              </View>
            </Card.Body>
            <Card.Footer
              content="footer content"
              extra="footer extra content"
            />
          </Card>

          <Card>
            <Card.Header
              title="This is title"
              thumbStyle={{ width: 30, height: 30 }}
              thumb="https://gw.alipayobjects.com/zos/rmsportal/MRhHctKOineMbKAZslML.jpg"
              extra="this is extra"
            />
            <Card.Body>
              <View style={{ height: 80 }}>
                <Text style={{ marginLeft: 16 }}>Card Content</Text>
              </View>
            </Card.Body>
            <Card.Footer
              content="footer content"
              extra="footer extra content"
            />
          </Card>

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
