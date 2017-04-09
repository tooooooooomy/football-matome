import React, { PropTypes } from 'react'
import {Table, TableBody, TableRow, TableRowColumn} from 'material-ui/Table';


const Feeds = ({feeds}) => (
  <Table>
    <TableBody displayRowCheckbox={false}>
    {feeds.map((feed, i) =>
      <TableRow key={i}>
        <TableRowColumn>
          <a href={feed.link}>{feed.title}</a>
        </TableRowColumn>
      </TableRow>
    )}
    </TableBody>
  </Table>
)

Feeds.propTypes = {
  feeds: PropTypes.array.isRequired,
}

export default Feeds
