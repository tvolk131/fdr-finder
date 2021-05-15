import {IconButton, Paper, InputBase} from '@material-ui/core';
import {makeStyles} from '@material-ui/core/styles';
import {Search as SearchIcon} from '@material-ui/icons';
import * as React from 'react';
import {MouseEvent, useState} from 'react';

const useStyles = makeStyles({
  root: {
    padding: '2px 4px',
    margin: 'auto',
    display: 'flex',
    alignItems: 'center',
    maxWidth: 800,
  },
  input: {
    marginLeft: 8,
    flex: 1,
  },
  iconButton: {
    padding: 10,
  },
  divider: {
    width: 1,
    height: 28,
    margin: 4,
  }
});

const SearchBar = () => {
  const [searchText, setSearchText] = useState('');

  const handleSearch = () => {
    if (searchText.length) {
      // TODO - Call search prop.
    }
  }

  const handleMouseDownSearch = (event: MouseEvent) => {
    event.preventDefault();
  };

  const classes = useStyles();

  return (
    <Paper className={classes.root}>
      <InputBase
        className={classes.input}
        placeholder="Search Freedomain Videos"
        value={searchText}
        onChange={(event) => {
          setSearchText(event.target.value);
        }}
        onKeyPress={(event) => {
          if (event.key === 'Enter') {
            handleSearch();
          }
        }}
        onSubmit={handleSearch}
      />
      <IconButton
        className={classes.iconButton}
        onMouseDown={handleMouseDownSearch}
        onClick={handleSearch}
      >
        <SearchIcon/>
      </IconButton>
    </Paper>
  );
};

export default SearchBar;