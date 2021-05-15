import {makeStyles} from '@material-ui/core/styles';
import * as React from 'react';
import SearchBar from '../components/searchBar';

const useStyles = makeStyles({
  searchBar: {
    margin: '10px',
    textAlign: 'center'
  }
});

const SearchPage = () => {
  const classes = useStyles();

  return (
    <div className={classes.searchBar}>
      <SearchBar/>
    </div>
  );
};

export default SearchPage;