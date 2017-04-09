import assert from 'power-assert'
import React from 'react'
import { shallow } from 'enzyme'

import Feeds from './Feeds'

const feeds = [
  {
    id: 1,
    title: 'hoge',
    link: 'http://www.google.co.jp'
  },
  {
    id: 2,
    title: 'fuga',
    link: 'http://www.google.co.jp'
  },
]

describe('Feeds', () => {
  it('return <ul>', () => {
    const wrapper = shallow(<Feeds feeds={feeds}/>)
    assert.equal(wrapper.find('ul').length, 1);
    assert.equal(wrapper.find('li').length, 2);
  })
})
