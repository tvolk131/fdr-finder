import {CircularProgress, Chip, Divider} from '@material-ui/core';
import {createStyles, makeStyles, Theme} from '@material-ui/core/styles';
import * as React from 'react';
import {useState, useEffect} from 'react';
import {getFilteredTagsWithCounts} from '../api';

const useStyles = makeStyles((theme: Theme) => (
  createStyles({
    root: {
      padding: '15px 5px 5px 15px'
    },
    divider: {
      margin: '5px 0'
    },
    tagChip: {
      margin: theme.spacing(0.5)
    }
  })
));

interface AdvancedSearchMenuProps {
  searchTags: string[]
  setSearchTags: (tags: string[]) => void
}

const AdvancedSearchMenu = ({searchTags, setSearchTags}: AdvancedSearchMenuProps) => {
  const [tagsWithCounts, setTagsWithCounts] = useState<{tag: string, count: number}[]>([]);
  const [isLoadingTags, setIsLoadingTags] = useState(true);

  const classes = useStyles();

  useEffect(() => {
    setSearchTags(searchTags);
    setIsLoadingTags(true);
    getFilteredTagsWithCounts(searchTags).then((tagsWithCounts) => {
      setTagsWithCounts(tagsWithCounts);
      setIsLoadingTags(false);
    });
  }, [searchTags]);

  return (
    <div>
      {!!searchTags.length && (
        <div>
          {searchTags.map((tag) => <Chip onDelete={() => setSearchTags(searchTags.filter((iterTag) => tag !== iterTag))} className={classes.tagChip} label={tag}/>)}
          <Divider className={classes.divider}/>
        </div>
      )}
      {isLoadingTags ? <CircularProgress/> : tagsWithCounts.sort((a, b) => {
        if (a.count < b.count) {
          return 1;
        } else if (a.count > b.count) {
          return -1;
        } else if (a.tag > b.tag) {
          return 1;
        } else if (a.tag < b.tag) {
          return -1;
        } else {
          return 0;
        }
      }).map(({tag, count}) => (
        <Chip onClick={() => setSearchTags([...searchTags, tag])} className={classes.tagChip} label={`${tag} (${count})`}/>
      ))}
    </div>
  );
};

export default AdvancedSearchMenu;