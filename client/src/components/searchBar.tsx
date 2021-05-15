import {IconButton, Paper, InputBase} from '@material-ui/core';
import {makeStyles} from '@material-ui/core/styles';
import {Search as SearchIcon} from '@material-ui/icons';
import * as React from 'react';
import {MouseEvent, useState} from 'react';

const useStyles = makeStyles({
  root: {
    padding: '2px 4px',
    display: 'flex',
    alignItems: 'center',
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

interface SearchBarProps {
  onSearch: (query: string) => void
  className?: string
}

const SearchBar = (props: SearchBarProps) => {
  const [searchText, setSearchText] = useState('');

  const handleSearch = () => {
    if (searchText.length) {
      props.onSearch(searchText);
    }
  }

  const handleMouseDownSearch = (event: MouseEvent) => {
    event.preventDefault();
  };

  const classes = useStyles();

  return (
    <Paper className={classes.root + (props.className ? ` ${props.className}` : '')}>
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