import {IconButton, InputBase, AccordionSummary, Accordion, AccordionDetails, Divider} from '@material-ui/core';
import {makeStyles} from '@material-ui/core/styles';
import {Search as SearchIcon} from '@material-ui/icons';
import * as React from 'react';
import {MouseEvent, useState} from 'react';
import AdvancedSearchMenu from '../components/advancedSearchMenu';
import ExpandMoreIcon from '@material-ui/icons/ExpandMore';

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
  onSearch: () => void
  searchText: string
  setSearchText: (query: string) => void
}

const SearchBar = (props: SearchBarProps) => {
  const handleSearch = () => {
    if (props.searchText.length) {
      props.onSearch();
    }
  }

  const handleMouseDownSearch = (event: MouseEvent) => {
    event.preventDefault();
  };

  const classes = useStyles();

  return (
    <Accordion>
      <AccordionSummary expandIcon={<ExpandMoreIcon/>}>
        <div
          onClick={(event) => event.stopPropagation()}
          onFocus={(event) => event.stopPropagation()}
          style={{
            width: '100%',
            display: 'flex'
          }}
        >
          <InputBase
            className={classes.input}
            placeholder="Search Freedomain Videos"
            value={props.searchText}
            onChange={(event) => {
              props.setSearchText(event.target.value);
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
        </div>
      </AccordionSummary>
      <Divider/>
      <AccordionDetails>
        <AdvancedSearchMenu/>
      </AccordionDetails>
    </Accordion>
  );
};

export default SearchBar;