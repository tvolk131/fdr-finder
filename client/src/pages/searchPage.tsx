import {Accordion, AccordionDetails, AccordionSummary, Typography} from '@material-ui/core';
import {makeStyles} from '@material-ui/core/styles';
import * as React from 'react';
import AdvancedSearchMenu from '../components/advancedSearchMenu';
import SearchBar from '../components/searchBar';
import ExpandMoreIcon from '@material-ui/icons/ExpandMore';

const useStyles = makeStyles({
  root: {
    margin: '10px',
    textAlign: 'center'
  },
  nested: {
    maxWidth: 800,
    margin: 'auto'
  },
  searchBar: {
    borderBottomLeftRadius: 0,
    borderBottomRightRadius: 0
  }
});

const SearchPage = () => {
  const classes = useStyles();

  return (
    <div className={classes.root}>
      <div className={classes.nested}>
        <SearchBar className={classes.searchBar} onSearch={(query) => console.log(query)}/>
        <Accordion>
          <AccordionSummary expandIcon={<ExpandMoreIcon/>}>
            <Typography>Advanced Search</Typography>
          </AccordionSummary>
          <AccordionDetails>
            <AdvancedSearchMenu/>
          </AccordionDetails>
        </Accordion>
      </div>
    </div>
  );
};

export default SearchPage;